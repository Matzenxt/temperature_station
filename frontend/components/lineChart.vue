<script lang="ts" setup>
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'
import { Line } from 'vue-chartjs'
import {Measurement} from "~/types/measurement";

  const props = defineProps<{
    measurements: Measurement[];
  }>();

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

  const labels: Array<string> = [];
  const temperatureData: Array<number> = [];
  const humidityData: Array<number> = [];

  for (const measurement of props.measurements) {
    labels.push(measurement.date_time.slice(0, 19).replace("T", " "));
    temperatureData.push(measurement.temperature);
    humidityData.push(measurement.humidity);
  }

  const chartData = {
    labels: labels,
    datasets: [
      {
        label: "Temperature",
        data: temperatureData,
        borderColor: '#eb3636',
        backgroundColor: '#f59b9b',
      },
      {
        label: "Humidity",
        data: humidityData,
        borderColor: '#36A2EB',
        backgroundColor: '#9BD0F5',
      }
    ]
  };
  const chartOptions = {
    responsive: true
  };

</script>

<template>
  <Line
      :options="chartOptions"
      :data="chartData"
  ></Line>
</template>

<style scoped>

</style>