<script lang="ts" setup>
  import {Measurement} from "~/types/measurement";
  import {$fetch} from "ofetch";
  import {useTempStationStore} from "~/store/tempstation";

  const config = useRuntimeConfig();
  const store = useTempStationStore();

  store.clearMeasurementData();

  const intervalSeconds: number = 15;
  const intervalTime: number = intervalSeconds * 1000;

  const diagramTimeMinutes = 60*4;

  const { room } = useRoute().params;

  const date = new Date();
  let dateTo = ref(date.toISOString().slice(0, 19));

  date.setMinutes(date.getMinutes() - diagramTimeMinutes);
  let dateFrom = ref(date.toISOString().slice(0, 19));

  let { data: measurements, pending, refresh} = await useAsyncData<Array<Measurement>>("measurements", () =>
      $fetch(config.public.url + '/measurement/' + room + '/' + dateTo.value + '/' + dateFrom.value, {
        method: 'GET',
      }));

  if (measurements.value != null) {
    //store.measurements = measurements.value;
    store.updateTempListe(measurements.value);
  }

  useIntervalFn(async () => {
        console.log("Refreshing room temperature and humidity.");

        const date = new Date();
        let to = date.toISOString().slice(0, 19);
        date.setMinutes(date.getMinutes() - diagramTimeMinutes);
        let from = date.toISOString().slice(0, 19);

        dateTo.value = to;
        dateFrom.value = from;

        refresh();

        if (measurements.value != null) {
          //store.measurements = measurements.value;
          store.updateTempListe(measurements.value);
        } else {
          console.log("null");
        }

      }, intervalTime
  );
</script>

<template>
  <div>
    <v-card>
      <v-card-title>{{ room }}</v-card-title>

      <LineChart v-if="store.measurements.length > 0"></LineChart>

      <v-card-text
          v-for="measurement in store.measurements" :key="measurement.id">
        Date: {{ measurement.date_time.slice(0, 19).replace('T', ' ') }} Device: {{ measurement.device }} - Temperature: {{ measurement.temperature }}°C - Humidity: {{ measurement.humidity }}%
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>