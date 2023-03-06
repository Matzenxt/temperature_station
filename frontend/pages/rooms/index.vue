<script lang="ts" setup>
  const config = useRuntimeConfig();

  console.log("Uri: " + config.public.url + ':' + config.public.port + '/room');
  const { data: rooms } = await useFetch(config.public.url + ':' + config.public.port + '/room');
  console.log("Rooms: " + rooms.value);
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