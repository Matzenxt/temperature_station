<script lang="ts" setup>
  import {Ref} from "vue";

  const baseIntervalSeconds: number = 120;
  const baseIntervalTime: number = baseIntervalSeconds * 1000;

  const intervalSeconds: Ref<number> = ref<number>(baseIntervalSeconds);
  const intervalTime: Ref<number> = ref<number>(baseIntervalTime);

  const config = useRuntimeConfig();

  let { data: rooms, pending, refresh } = useFetch(config.public.url + '/room', {
    method: 'GET',
  });

  const {pause, resume, isActive} = useIntervalFn(refreshRoomList, intervalTime);

  async function refreshRoomList() {
    console.log("Refreshing room list.");
    await refresh();
  }

  function updateIntervalTime() {
    intervalTime.value = intervalSeconds.value * 1000;
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

        <v-text-field
            label="Intervall"
            hint="Aktualisierungs Intervall"
            v-model="intervalSeconds"
            type="number"
            suffix="sec"
            @change="updateIntervalTime()"
        />
      </v-row>
    </v-container>

    <v-card v-for="room in rooms" :key="room">
      <v-card-title><NuxtLink :to="'/rooms/' + room">{{ room }}</NuxtLink></v-card-title>
    </v-card>
  </v-container>
</template>

<style scoped>

</style>