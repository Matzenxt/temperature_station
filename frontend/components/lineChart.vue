<script lang="ts" setup>
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  type ChartData
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

  let labels: Array<string> = Array<string>();
  let temperatureData: Array<number> =Array<number>();
  let humidityData: Array<number> = Array<number>();

  for (const measurement of store.measurements) {
    let date = new Date(measurement.date_time);

    labels.push(date.toLocaleString());
    temperatureData.push(measurement.temperature);
    humidityData.push(measurement.humidity);
  }

  const data = ref<ChartData<'line'>>({
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
  });

  const chartOptions = {
    responsive: true
  };

  setInterval(() => {
    labels = [];
    temperatureData = [];
    humidityData = [];

    for (const measurement of store.measurementData) {
      let date = new Date(measurement.date_time);

      labels.push(date.toLocaleString());
      temperatureData.push(measurement.temperature);
      humidityData.push(measurement.humidity);
    }

    data.value = {
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

  }, 5000);

</script>

<template>
  <Line
      :options="chartOptions"
      :data="data"
  ></Line>

</template>

<style scoped>

</style>