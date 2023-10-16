<script setup lang="ts">
  import DashboardItem from "~/pages/dashboard/dashboardItem.vue";
  import DashboardSettings from "~/pages/dashboard/dashboardSettings.vue";

  const config = useRuntimeConfig();

  let rooms: string[] = [];
  let avgTimeSeconds: number = 900;
  let updateInterval: number = 120 * 1000;

  if (process.client) {
    const content = localStorage.getItem("rooms");
    const avgTime = localStorage.getItem("dashboard_avgTimeSeconds");
    const updateIntervallTimeSeconds = localStorage.getItem("dashboard_updateIntervalTimeSeconds");

    if (content != null) {
      rooms = JSON.parse(content);
      console.log("Loaded rooms: " + rooms);
    }

    if (avgTime != null) {
      avgTimeSeconds = parseInt(avgTime);
      console.log("Loaded avg time: " + avgTimeSeconds);
    }

    if (updateIntervallTimeSeconds != null) {
      updateInterval = parseInt(updateIntervallTimeSeconds) * 1000;
      console.log("Loaded update intervall time: " + updateIntervallTimeSeconds);
    }
  }

  let {data: dashboardItems, pending, refresh} = useFetch(config.public.url + '/dashboard/items/' + avgTimeSeconds, {
    method: 'POST',
    body: JSON.stringify(rooms)
  });
  const {pause, resume, isActive} = useIntervalFn(updateDashboardDate, updateInterval);

  async function updateDashboardDate() {
    console.log("Refreshing dashboard data.");
    await refresh();
  }

</script>

<template>
  <v-card>
    <v-card-title>
      Dashboard
      <DashboardSettings/>

      <v-progress-circular
          v-if="pending"
          indeterminate
          color="#9c9a9a"
          size="30"
      ></v-progress-circular>

    </v-card-title>

    <v-divider/>

    <v-card-text>
      <v-row no-gutters>
        <v-col
            v-for="dashboardItem in dashboardItems"
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