<script setup lang="ts">
  import {Ref, ref} from 'vue';
  const config = useRuntimeConfig();

  let dialog: Ref<boolean> = ref<boolean>(false);
  let selectedRooms: Ref<string[]> = ref<string[]>([]);

  let { data: rooms, pending, refresh } = useFetch<string[]>(config.public.url + '/room', {
    method: 'GET',
  });

  let roomList: string[];

  if (rooms.value != null) {
    roomList = rooms.value;
  } else {
    roomList = [];
  }

  if (process.client) {
    const content = localStorage.getItem("rooms");

    if (content != null) {
      selectedRooms.value = JSON.parse(content);
    }
  }

  function saveSettings() {
    if (process.client) {
      localStorage.setItem("rooms", JSON.stringify(selectedRooms.value));
    }

    closeDialog();
  }

  function closeDialog() {
    dialog.value = false;
  }
</script>

<template>
  <v-dialog
      v-model="dialog"
      width="800px"
  >
    <template v-slot:activator="{ props }">
      <v-btn
          v-bind="props"
      >
        <v-icon>mdi-cog</v-icon>
      </v-btn>
    </template>

    <v-card>
      <v-card-title>Dashboard Einstellungen</v-card-title>

      <v-divider/>

      <v-card-text>

        <v-autocomplete
            label="Räume"
            :items="roomList"
            v-model="selectedRooms"
            chips
            multiple
        ></v-autocomplete>

      </v-card-text>

      <v-divider/>

      <v-card-actions>
        <v-spacer/>
        <v-btn
            @click="closeDialog()"
        >
          Abbrechen
        </v-btn>
        <v-btn
            @click="saveSettings()"
        >
          Speichern
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>

</style>