<script setup lang="ts">
  import {DashboardItemData} from "~/types/dashboardItem";

  const props = defineProps({
    dashboardItem: {type: DashboardItemData, required: true},
  });

  const dashboardItem: DashboardItemData = props.dashboardItem;
</script>

<template>
  <div>
    Temp:
    Hum:
    AVG Temp: {{dashboardItem.avg_temperature}}
    AVG: Hum: {{dashboardItem.avg_humidity}}
  </div>
</template>

<style scoped>

</style>