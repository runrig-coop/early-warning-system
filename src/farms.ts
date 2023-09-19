import { StatusObject } from "./status";

export interface BackendFarmObject {
  id: number,
  name: string,
  status: string,
  timestamp: number,
}
export interface FarmListItem {
  id: number,
  name: string,
  status: StatusObject,
  timestamp: number,
}

export const toBackendFarmObject = (f: FarmListItem): BackendFarmObject =>
  ({ ...f, status: f.status.title });
