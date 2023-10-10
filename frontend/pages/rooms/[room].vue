<script lang="ts" setup>
  import {Measurement} from "~/types/measurement";
  import {$fetch} from "ofetch";
  import {useTempStationStore} from "~/store/tempstation";
  import {Ref} from "vue";

  const config = useRuntimeConfig();
  const store = useTempStationStore();

  store.clearMeasurementData();

  const baseIntervalSeconds: number = 120;
  const baseIntervalTime: number = baseIntervalSeconds * 1000;

  const intervalSeconds: Ref<number> = ref<number>(baseIntervalSeconds);
  const intervalTime: Ref<number> = ref<number>(baseIntervalTime);

  const diagramTimeMinutes = 60*24*1;

  const { room } = useRoute().params;

  const date = new Date();
  let startString = "Von";
  let dateTo = ref(date.toISOString().slice(0, 19));

  date.setMinutes(date.getMinutes() - diagramTimeMinutes);
  const endString = "Bis";
  let dateFrom = ref(date.toISOString().slice(0, 19));

  let { data: measurements, pending, refresh} = await useAsyncData<Array<Measurement>>("measurements", () =>
      $fetch(config.public.url + '/measurement/' + room + '/' + dateTo.value + '/' + dateFrom.value, {
        method: 'GET',
      }));

  if (measurements.value != null) {
    //store.measurements = measurements.value;
    store.updateTempListe(measurements.value);
  }

  const {pause, resume, isActive} = useIntervalFn(getMeasurePoints, intervalTime);
  
  async function getMeasurePoints() {
    console.log("Refreshing room temperature and humidity.");

    const date = new Date();
    let to = date.toISOString().slice(0, 19);
    date.setMinutes(date.getMinutes() - diagramTimeMinutes);
    let from = date.toISOString().slice(0, 19);

    dateTo.value = to;
    dateFrom.value = from;

    await refresh();

    if (measurements.value != null) {
      //store.measurements = measurements.value;
      store.updateTempListe(measurements.value);
    } else {
      console.log("null");
    }
  }

  function updateIntervalTime() {
    intervalTime.value = intervalSeconds.value * 1000;
  }
</script>

<template>
  <div>
    <v-card>
      <v-card-title>
        <v-btn
            density="comfortable"
            icon="mdi-refresh"
            @click="getMeasurePoints()"
            :disabled="pending"
        >
        </v-btn>

        {{ room }}

        <v-progress-circular
            v-if="pending"
            indeterminate
            color="#9c9a9a"
            size="30"
        ></v-progress-circular>
      </v-card-title>

      <v-divider/>

      <v-card-text>
        <v-text-field
            label="Intervall"
            hint="Aktualisierungs Intervall"
            v-model="intervalSeconds"
            type="number"
            suffix="sec"
            @change="updateIntervalTime()"
        />
        <DateTimePicker
            v-bind:text="startString"
            v-model:date="date"
        />

        <LineChart v-if="store.measurements.length > 0"></LineChart>

        <v-expansion-panels>
          <v-expansion-panel>
            <v-expansion-panel-title>Messdaten</v-expansion-panel-title>
            <v-expansion-panel-text>
              <p
                  v-for="measurement in store.measurements" :key="measurement.id">
                Date: {{ new Date(measurement.date_time).toLocaleString() }} Device: {{ measurement.device }} - Temperature: {{ measurement.temperature }}°C - Humidity: {{ measurement.humidity }}%
              </p>
            </v-expansion-panel-text>
          </v-expansion-panel>
        </v-expansion-panels>

      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>