
interface Measurement {
  id: number;
  room: string;
  device: string;
  date_time: string;
  temperature: number;
  humidity: number;
}

export default Measurement;
