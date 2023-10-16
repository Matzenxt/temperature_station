<script setup lang="ts">
  import {DashboardItemData} from "~/types/dashboardItemData";

  const props = defineProps({
    dashboardItem: {type: DashboardItemData, required: true},
  });

  const dashboardItem: DashboardItemData = props.dashboardItem;
</script>

<template>
  <v-card>
    <v-card-title>{{dashboardItem.room_name}}</v-card-title>

    <v-divider/>

    <v-card-text>

      <p>Aktuelle Werte:</p>
      {{ new Date(dashboardItem.last_time).toLocaleString() }}
      <v-row>
        <v-col>
          <v-progress-circular
              :rotate="0"
              :size="100"
              :width="15"
              :model-value="dashboardItem.temperature"
              color="red"
          >
            {{ dashboardItem.temperature.toPrecision(4) }}°C
          </v-progress-circular>
        </v-col>
        <v-col>
          <v-progress-circular
              :rotate="0"
              :size="100"
              :width="15"
              :model-value="dashboardItem.humidity"
              color="blue"
          >
            {{ dashboardItem.humidity.toPrecision(4) }}%
          </v-progress-circular>
        </v-col>
      </v-row>

      <p>Durchschnittswerte:</p>
      <v-row>
        <v-col>
          <v-progress-circular
              :rotate="0"
              :size="100"
              :width="15"
              :model-value="dashboardItem.avg_temperature"
              color="red"
              :disabled="true"
          >
            {{ dashboardItem.avg_temperature.toPrecision(4) }}°C
          </v-progress-circular>
        </v-col>
        <v-col>
          <v-progress-circular
              :rotate="0"
              :size="100"
              :width="15"
              :model-value="dashboardItem.avg_humidity"
              color="blue"
              :disabled="true"
          >
            {{ dashboardItem.avg_humidity.toPrecision(4) }}%
          </v-progress-circular>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<style scoped>

</style>