<script setup>
  const { data: rooms } = await useFetch('http://192.168.1.104:9090/room');
</script>

<template>
  <div>
    <h1>Raum übersicht</h1>
    <v-card v-for="room in rooms">
      <v-card-title><NuxtLink :to="'/rooms/' + room">{{ room }}</NuxtLink></v-card-title>
    </v-card>
  </div>
</template>

<style scoped>

</style>