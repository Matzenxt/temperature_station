<script lang="ts" setup>
  import {Measurement} from "~/types/measurement";

  const intervalSeconds: number = 5;
  const intervalTime: number = intervalSeconds * 1000;

  const { room } = useRoute().params;

  const dateTo = new Date();
  const dateFrom = new Date();
  dateFrom.setMinutes(dateTo.getMinutes() - 30);

  let toString = dateTo.toISOString().slice(0, 19);
  let fromString = dateFrom.toISOString().slice(0, 19);

  const uri = 'http://localhost:9090/measurement/'
      + room + "/" + toString + "/" + fromString;

  let { data: measurements } = await useFetch<Array<Measurement>>(uri, {key: room.toString()});

  useIntervalFn(async () => {
        console.log("refreshing");

        // TODO: Add functionality to update data and ui

      }, intervalTime
  );
</script>

<template>
  <div>
    <v-card>
      <v-card-title>{{ room }}</v-card-title>

      <LineChart v-if="measurements != null" :measurements="measurements"></LineChart>

      <v-card-text
          v-for="measurement in measurements" :key="measurement.id">
        Temperature: {{ measurement.temperature }}°C - Humidity: {{ measurement.humidity }}%
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>