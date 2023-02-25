import {defineStore} from "pinia";
import {Measurement} from "~/types/measurement";

interface State {
  measurements: Array<Measurement>,
  labels: Array<string>,
  temperatureData: Array<number>,
  humidityData: Array<number>,
}

export const useTempStationStore = defineStore({
  id: 'temp-station-store',
  state: (): State => ({
    measurements: [],
    labels: [],
    temperatureData: [],
    humidityData: [],
  }),
  actions: {
    updateTempListe(measurements: Array<Measurement>) {
      this.measurements = measurements;

      /*
      this.labels = [];
      this.temperatureData = [];
      this.humidityData = [];

      for (const measurement of this.measurements) {
        this.labels.push(measurement.date_time.slice(0, 19).replace("T", " "));
        this.temperatureData.push(measurement.temperature);
        this.humidityData.push(measurement.humidity);
      }
       */
    }
  },
  getters: {
    tempDataLineChart: (state): [Array<string>, Array<number>, Array<number>] => {
      const labels: Array<string> = [];
      const temperatureData: Array<number> = [];
      const humidityData: Array<number> = [];

      for (const measurement of state.measurements) {
        labels.push(measurement.date_time.slice(0, 19).replace("T", " "));
        temperatureData.push(measurement.temperature);
        humidityData.push(measurement.humidity);
      }

      return [labels, temperatureData, humidityData];
    },

    measurementData: (state) => {
      return state.measurements;
    },
  },
});
