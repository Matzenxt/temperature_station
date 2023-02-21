<script lang="ts" setup>
  import {Measurement} from "~/types/measurement";

  const config = useRuntimeConfig();

  const intervalSeconds: number = 5;
  const intervalTime: number = intervalSeconds * 1000;

  const { room } = useRoute().params;

  const dateTo = new Date();
  const dateFrom = new Date();
  dateFrom.setMinutes(dateTo.getMinutes() - 5);

  let toString = dateTo.toISOString().slice(0, 19);
  let fromString = dateFrom.toISOString().slice(0, 19);

  const uri = config.public.url + ':' + config.public.port + '/measurement/'
      + room + "/" + toString + "/" + fromString;

  console.log("Uri:" + uri);

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