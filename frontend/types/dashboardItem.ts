export class DashboardItemData {
    room_id: number;
    room_name: string;
    last_time: string;
    temperature: number;
    humidity: number;
    avg_duration_seconds: number;
    avg_temperature: number;
    avg_humidity: number;

    constructor(room_id: number, room_name: string, last_time: string, temperature: number, humidity: number, avg_duration_seconds: number, avg_temperature: number, avg_humidity: number) {
        this.room_id = room_id;
        this.room_name = room_name;
        this.last_time = last_time;
        this.temperature = temperature;
        this.humidity = humidity;
        this.avg_duration_seconds = avg_duration_seconds;
        this.avg_temperature = avg_temperature;
        this.avg_humidity = avg_humidity;
    }
}
