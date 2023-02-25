<script lang="ts" setup>
  import {Measurement} from "~/types/measurement";
  import {$fetch} from "ofetch";
  import {Ref} from "vue";

  const config = useRuntimeConfig();

  const intervalSeconds: number = 5;
  const intervalTime: number = intervalSeconds * 1000;

  const diagramTimeMinutes = 200;

  const { room } = useRoute().params;

  const date = new Date();
  let dateTo = ref(date.toISOString().slice(0, 19));

  date.setMinutes(date.getMinutes() - diagramTimeMinutes);
  let dateFrom = ref(date.toISOString().slice(0, 19));

  let { data: measurements, pending, refresh} = await useAsyncData<Array<Measurement>>("measurements", () =>
      $fetch(config.public.url + ':' + config.public.port + '/measurement/' + room + '/' + dateTo.value + '/' + dateFrom.value ));

  let points: Ref<Array<Measurement>> = ref([]);

  if (measurements.value != null) {
    points.value = measurements.value;
  }

  useIntervalFn(async () => {
        console.log("refreshing");

        const date = new Date();
        let to = date.toISOString().slice(0, 19);
        date.setMinutes(date.getMinutes() - diagramTimeMinutes);
        let from = date.toISOString().slice(0, 19);

        dateTo.value = to;
        dateFrom.value = from;

        refresh();

        if (measurements.value != null) {
          points.value = measurements.value;
        }

      }, intervalTime
  );
</script>

<template>
  <div>
    <v-card>
      <v-card-title>{{ room }}</v-card-title>

      <LineChart v-if="measurements != null" :measurements="measurements"></LineChart>

      <v-card-text
          v-for="measurement in points" :key="measurement.id">
        Date: {{ measurement.date_time.slice(0, 19).replace('T', ' ') }} Device: {{ measurement.device }} - Temperature: {{ measurement.temperature }}°C - Humidity: {{ measurement.humidity }}%
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>