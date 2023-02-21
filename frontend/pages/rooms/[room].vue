<script lang="ts" setup>

  import {Measurement} from "~/types/measurement";

  const { room } = useRoute().params;

  const dateTo = new Date();
  const dateFrom = new Date();
  dateFrom.setMinutes(dateTo.getMinutes() - 30);

  const toString = dateTo.toISOString().slice(0, 19);
  const fromString = dateFrom.toISOString().slice(0, 19);

  const uri = 'http://192.168.1.104:9090/measurement/'
      + room + "/" + toString + "/" + fromString;

  const { data: measurements } = await useFetch<Array<Measurement>>(uri, {key: room.toString()});
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