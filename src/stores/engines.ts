import { defineStore } from 'pinia'

type Variant =
    | 'chess'
    | 'crazyhouse'
    | 'antichess'
    | 'atomic'
    | 'horde'
    | 'kingofthehill'
    | 'racingkings'
    | '3check'

export type Engine = {
    id?: string
    name: string
    maxThreads: number
    maxHash: number
    defaultDepth: number
    variants: Variant[]
    providerSecret: string
    providerData?: string

    binaryLocation: string
}

export const useEnginesStore = defineStore('engines', {
    state: () => ({
        engines: [] as Engine[],
    }),
    actions: {
        addEngine(engine: Engine) {
            this.engines.push(engine)

            return engine
        },
        findById(id: string) {
            return this.engines.find((e) => e.id === id)
        },
    },
    persist: true,
})
