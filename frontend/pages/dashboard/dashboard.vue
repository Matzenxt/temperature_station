<script setup lang="ts">
  import DashboardItem from "~/pages/dashboard/dashboardItem.vue";
  import DashboardSettings from "~/pages/dashboard/dashboardSettings.vue";
  import {DashboardItemData} from "~/types/dashboardItem";
  import {getAllDashboardItems} from "~/network/dashboard";

  let rooms: string[] = [];
  let avgTimeSeconds: number = 900;

  if (process.client) {
    const content = localStorage.getItem("rooms");
    const avgTime = localStorage.getItem("dashboard_avgTimeSeconds");

    if (content != null) {
      rooms = JSON.parse(content);
      console.log("Loaded rooms: " + rooms);
    }

    if (avgTime != null) {
      avgTimeSeconds = parseInt(avgTime);
      console.log("Loaded avg time: " + avgTimeSeconds);
    }
  }

  let dashboardItems: DashboardItemData[] = await getAllDashboardItems(rooms, avgTimeSeconds);

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
            v-for="dashboardItem in dashboardItems"
            :key="dashboardItem.room_id"
            cols="12"
            sm="4"
        >
          <v-sheet class="ma-2 pa-2">
            <DashboardItem
                v-bind:dashboard-item="dashboardItem"
            />
          </v-sheet>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<style scoped>

</style>