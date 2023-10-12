<script setup lang="ts">
  import DashboardItem from "~/dashboard/dashboardItem.vue";
  import DashboardSettings from "~/dashboard/dashboardSettings.vue";

  let rooms: string[] = [];
  if (process.client) {
    const content = localStorage.getItem("rooms");

    if (content != null) {
      rooms = JSON.parse(content);
      console.log("Loaded rooms: " + rooms);
    }
  }

</script>

<template>
  <v-card>
    <v-card-title>
      Dashboard
      <DashboardSettings/>
    </v-card-title>

    <v-divider/>

    <v-card-text>
      <v-row no-gutters>
        <v-col
            v-for="n in rooms.length"
            :key="n"
            cols="12"
            sm="4"
        >
          <v-sheet class="ma-2 pa-2">
            <DashboardItem/>
          </v-sheet>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<style scoped>

</style>