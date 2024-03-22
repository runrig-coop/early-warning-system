import { BaseDirectory, createDir, exists, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
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
  const cacheDir = '.cache';
  const cachePath = `${cacheDir}/early_warning.json`;
  const cacheOpts: { dir: number } = { dir: BaseDirectory.Resource };
  const cacheDirOpts = { ...cacheOpts, recursive: true };
  const farms: FarmListItem[] = reactive([]);

  onMounted(async () => {
    try {
      if (!exists(cacheDir, cacheOpts)) {
        await createDir(cacheDir, cacheDirOpts);
      }
      const json = await readTextFile(cachePath, cacheOpts);
      const { farms: cache } = JSON.parse(json);
      const list: FarmListItem[] = cache.map((f) => ({
        ...f, status: toStatusObject(f.status),
      }));
      farms.push(...list);
    } catch (error) {
      console.log('No cache found. Error message: ', error.message);
    }
  })

  async function save() {
    const cache: BackendFarmObject[] = farms.map(toBackendFarmObject);
    const json = JSON.stringify({ farms: cache }, null, 2);
    return writeTextFile(cachePath, json, cacheOpts)
      .then(() => { console.log("Write Success!"); })
      .catch(console.error);
  }

  return { farms, save };
}

export const examples: FarmListItem[] = [
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
