<script lang="ts" setup>
  const intervalSeconds: number = 10;
  const intervalTime: number = intervalSeconds * 1000;

  const config = useRuntimeConfig();

  let { data: rooms, pending, refresh } = useFetch(config.public.url + '/room', {
    method: 'GET',
  });

  useIntervalFn(refreshRoomList, intervalTime);

  async function refreshRoomList() {
    console.log("Refreshing room list.");

    await refresh();
  }
</script>

<template>
  <v-container>
    <v-container>
      <v-row>
        <v-btn
            density="comfortable"
            icon="mdi-refresh"
            @click="refreshRoomList()"
            :disabled="pending"
        >
        </v-btn>

        <h1>
          Raum übersicht
        </h1>

        <v-progress-circular
            v-if="pending"
            indeterminate
            color="#9c9a9a"
            size="30"
        ></v-progress-circular>
      </v-row>
    </v-container>

    <v-card v-for="room in rooms" :key="room">
      <v-card-title><NuxtLink :to="'/rooms/' + room">{{ room }}</NuxtLink></v-card-title>
    </v-card>
  </v-container>
</template>

<style scoped>

</style>