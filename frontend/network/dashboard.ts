import {requestOptionsPOST} from "~/network/network";
import {DashboardItemData} from "~/types/dashboardItem";

export async function getAllDashboardItems(rooms: String[], avgSeconds: number): Promise<Array<DashboardItemData>> {
    const config = useRuntimeConfig();

    const requestOption = requestOptionsPOST(rooms);
    let resp = await fetch(config.public.url + ":" + config.public.port + "/dashboard/items/" + avgSeconds, requestOption);
    let text = await resp.text();

    if (resp.ok) {
        let labels: Array<DashboardItemData> = JSON.parse(text);
        return labels;
    } else {
        return [];
    }
}
