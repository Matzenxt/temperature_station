<script setup>
  const { room } = useRoute().params;

  const uri = 'http://192.168.1.104:9090/measurement/'
      + room + "/2000-02-16/2023-12-20";

  const { data: measurements } = await useFetch(uri, {key: room});
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