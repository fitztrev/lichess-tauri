import { Chess, Move, parseUci } from 'chessops'
import { parseFen } from 'chessops/fen'
import { makeSanAndPlay } from 'chessops/san'
import { defineStore } from 'pinia'

export interface LichessWorkEvent {
  id: number
  event: string
  windowLabel: string
  payload: {
    event: 'Status' | 'Uci'
    message: string
    analysis_request: false
  }
}

interface UciDetails {
  depth?: string
  seldepth?: string
  multipv?: string
  score?: {
    type?: string
    value?: string
  }
  nodes?: string
  nps?: string
  hashfull?: string
  tbhits?: string
  time?: string
  pv?: string[]
  bestmove?: string
  ponder?: string
}

interface LichessAnalysisRequest {
  id: string
  engine: {
    clientSecret: string
    defaultDepth: number
    id: string
    maxHash: number
    maxThreads: number
    name: string
    providerData: null
    userId: string
    variants: string[]
  }
  work: {
    hash: number
    infinite: boolean
    initialFen: string
    moves: string[]
    multiPv: number
    sessionId: string
    threads: number
    variant: string
  }
}

export const useAnalysisStore = defineStore('event-logs', {
  state: () => ({
    status: '',
    request: {} as LichessAnalysisRequest,
    uci: {} as UciDetails,
  }),
  getters: {
    nodes: (state) =>
      convertNumberToUnitString(parseInt(state.uci.nodes || '')),
    nps: (state) => convertNumberToUnitString(parseInt(state.uci.nps || '')),
    fen: (state) => {
      if (state.request.work.moves.length === 0) {
        return state.request.work.initialFen
      }

      // const setup = parseFen(state.request.work.initialFen).unwrap()
      // const pos = Chess.fromSetup(setup).unwrap()

      // const move = state.request.work.moves[0]

      // const san = makeSanAndPlay(pos, parseUci(move) as Move)
    },
  },
  actions: {
    add(event: LichessWorkEvent) {
      if (event.payload.event === 'Status') {
        this.status = event.payload.message
      } else if (event.payload.event === 'Uci') {
        this.uci = { ...this.uci, ...parseUciString(event.payload.message) }
      }

      if (event.payload.analysis_request) {
        this.request = event.payload.analysis_request
      }
    },
  },
})

function convertNumberToUnitString(number: number): string {
  if (number < 1000) {
    return number.toString()
  } else if (number < 1_000_000) {
    return `${(number / 1000).toFixed(1)}k`
  } else if (number < 1_000_000_000) {
    return `${(number / 1_000_000).toFixed(1)}M`
  } else {
    return `${(number / 1_000_000_000).toFixed(1)}B`
  }
}

function parseUciString(uci: string): UciDetails {
  const stringType = uci.split(' ')[0]

  if (stringType === 'bestmove') {
    return {
      bestmove: uci.match(/bestmove (\w+)/)?.[1],
      ponder: uci.match(/ponder (\w+)/)?.[1],
    }
  }

  return {
    depth: uci.match(/depth (\d+)/)?.[1],
    seldepth: uci.match(/seldepth (\d+)/)?.[1],
    multipv: uci.match(/multipv (\d+)/)?.[1],
    score: {
      type: uci.match(/score (\w+)/)?.[1],
      value: uci.match(/score \w+ (\d+)/)?.[1],
    },
    nodes: uci.match(/nodes (\d+)/)?.[1],
    nps: uci.match(/nps (\d+)/)?.[1],
    hashfull: uci.match(/hashfull (\d+)/)?.[1],
    tbhits: uci.match(/tbhits (\d+)/)?.[1],
    time: uci.match(/time (\d+)/)?.[1],
    pv: uci.match(/ pv (.+)/)?.[1].split(' '),
  }
}

if (import.meta.vitest) {
  const { expect, test } = import.meta.vitest
  test.each([
    [100, '100'],
    [1_090, '1.1k'],
    [800_403, '800.4k'],
    [3_123_456, '3.1M'],
    [1_789_123_456, '1.8B'],
  ])('converts numbers to string representation', (number, withUnits) => {
    expect(convertNumberToUnitString(number)).toStrictEqual(withUnits)
  })

  test.each([
    [
      'bestmove b4c5 ponder b6c7',
      {
        bestmove: 'b4c5',
        ponder: 'b6c7',
      },
    ],
    [
      'info depth 26 seldepth 36 multipv 1 score cp 209 nodes 21219470 nps 4641178 hashfull 1000 tbhits 0 time 4572 pv d1b3 d8e7',
      {
        depth: '26',
        seldepth: '36',
        multipv: '1',
        score: {
          type: 'cp',
          value: '209',
        },
        nodes: '21219470',
        nps: '4641178',
        hashfull: '1000',
        tbhits: '0',
        time: '4572',
        pv: ['d1b3', 'd8e7'],
      },
    ],
  ])('parses uci strings', (uci, expected) => {
    expect(parseUciString(uci)).toStrictEqual(expected)
  })
}
