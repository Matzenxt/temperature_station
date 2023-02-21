<script lang="ts" setup>

  import {Measurement} from "~/types/measurement";

  const { room } = useRoute().params;

  const dateTo = new Date();
  const dateFrom = new Date();
  dateFrom.setMinutes(dateTo.getMinutes() - 30);

  const toString = dateTo.toISOString().slice(0, 19);
  const fromString = dateFrom.toISOString().slice(0, 19);

  const test = 'http://192.168.1.104:9090/measurement/'
      + room + "/" + toString + "/" + fromString;

  console.log("BLA: " + test);

  const uri = 'http://192.168.1.104:9090/measurement/'
      + room + "/2000-02-17 00:15:00/2023-02-17 00:00:00";

  const { data: measurements } = await useFetch<Array<Measurement>>(test, {key: room.toString()});

  console.log(measurements);
</script>

<template>
  <div>
    <p>Raum {{ room }}</p>

    <v-card v-for="measurement in measurements" :key="measurement.id">
      <v-card-title>{{ measurement.room }}</v-card-title>
      <v-card-text>Temp: {{ measurement.temperature }}</v-card-text>
    </v-card>
  </div>
</template>

<style scoped>

</style>