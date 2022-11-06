import { defineStore } from "pinia";

export interface WorkRequest {
  id: string;
  work: {
    sessionId: string;
    threads: number;
    hash: number;
    infinite: boolean;
    multiPv: number;
    variant: string;
    initialFen: string;
    moves: any[];
  };
  engine: {
    id: string;
    name: string;
    clientSecret: string;
    userId: string;
    maxThreads: number;
    maxHash: number;
    defaultDepth: number;
    variants: string[];
    providerData: null;
  };
}

export const useWorkRequestsStore = defineStore("work-requests", {
  state: () => ({
    requests: [] as WorkRequest[],
  }),
  actions: {
    add(workRequest: WorkRequest) {
      this.requests.push(workRequest);
    },
  },
});
