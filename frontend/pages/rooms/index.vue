<script lang="ts" setup>
  const intervalSeconds: number = 30;
  const intervalTime: number = intervalSeconds * 1000;

  const config = useRuntimeConfig();

  const { data: rooms, pending, refresh } = await useFetch(config.public.url + '/room', {
    method: 'GET',
  });

  useIntervalFn(async () => {
        console.log("Refreshing room list.");

        refresh();

      }, intervalTime
  );
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