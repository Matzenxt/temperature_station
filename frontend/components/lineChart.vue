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
  import {useTempStationStore} from "~/store/tempstation";

  ChartJS.register(
      CategoryScale,
      LinearScale,
      PointElement,
      LineElement,
      Title,
      Tooltip,
      Legend
  );

  const store = useTempStationStore();

  const labels: Array<string> = [];
  let temperatureData: Array<number> = [];
  let humidityData: Array<number> = [];

  for (const measurement of store.measurements) {
    labels.push(measurement.date_time.slice(0, 19).replace("T", " "));
    temperatureData.push(measurement.temperature);
    humidityData.push(measurement.humidity);
  }

  let chartData = {
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

  setInterval(() => {
    temperatureData = [];
    humidityData = [];

    for (const measurement of store.measurementData) {
      temperatureData.push(measurement.temperature);
      humidityData.push(measurement.humidity);
    }
  }, 1000);

</script>

<template>
  <Line
      :options="chartOptions"
      :data="chartData"
  ></Line>
</template>

<style scoped>

</style>