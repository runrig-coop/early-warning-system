import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, reactive } from 'vue';
import { StatusObject, toStatusObject, RED, YELLOW, GREEN } from './status';

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

const toBackendFarmObject = (f: FarmListItem): BackendFarmObject =>
  ({ ...f, status: f.status.title });

export default function useFarms() {
  const farms: FarmListItem[] = reactive([]);
  onMounted(() => {
    invoke('load').then((cache: BackendFarmObject[]): void => {
      const list: FarmListItem[] = cache.map((f) => ({
        ...f, status: toStatusObject(f.status),
      }));
      farms.push(...list);
    });
  })

  function save() {
    const cache: BackendFarmObject[] = farms.map(toBackendFarmObject);
    invoke('save', { farms: cache });
  }

  function setFarmStatus(i: number, color: string|symbol): void {
    farms[i].status = toStatusObject(color);
  }


  const examples: FarmListItem[] = [
    {
      id: 0,
      name: 'Joe\'s Farm',
      status: toStatusObject(RED),
      timestamp: 20
    },
    {
      id: 1,
      name: 'Sally\'s Farm',
      status: toStatusObject(YELLOW),
      timestamp: 5
    },
    {
      id: 2,
      name: 'Sam\'s Farm',
      status: toStatusObject(GREEN),
      timestamp: 0
    },
  ];

  return {
    examples, farms, save, setFarmStatus,
  }
}
