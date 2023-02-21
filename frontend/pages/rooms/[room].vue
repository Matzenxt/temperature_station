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

  console.log(measurements);
</script>

<template>
  <div>
    <p>Raum {{ room }}</p>

    <v-card v-for="measurement in measurements" :key="measurement.id">
      <v-card-title>{{ measurement.room }}</v-card-title>
      <v-card-text>Temperature: {{ measurement.temperature }}°C - Humidity: {{ measurement.humidity }}%</v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>