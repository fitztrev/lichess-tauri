import { Chess, Move, parseUci } from 'chessops'
import { makeFen, parseFen } from 'chessops/fen'
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
    fen: (state) =>
      generateFenFromMoves(
        state.request.work?.initialFen,
        state.request.work?.moves
      ),
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
      value: uci.match(/score \w+ (-?\d+)/)?.[1],
    },
    nodes: uci.match(/nodes (\d+)/)?.[1],
    nps: uci.match(/nps (\d+)/)?.[1],
    hashfull: uci.match(/hashfull (\d+)/)?.[1],
    tbhits: uci.match(/tbhits (\d+)/)?.[1],
    time: uci.match(/time (\d+)/)?.[1],
    pv: uci.match(/ pv (.+)/)?.[1].split(' '),
  }
}

function generateFenFromMoves(initialFen: string, moves: string[]): string {
  if (!initialFen)
    return 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'

  if (moves.length === 0) {
    return initialFen
  }

  const setup = parseFen(initialFen).unwrap()
  const pos = Chess.fromSetup(setup).unwrap()

  let fen = initialFen
  for (const move of moves) {
    makeSanAndPlay(pos, parseUci(move) as Move)
    fen = makeFen(pos.toSetup())
  }

  return fen
}

if (import.meta.vitest) {
  const { expect, test } = import.meta.vitest

  test.each([
    [
      'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
      ['e2e4', 'e7e5'],
      'rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2',
    ],
  ])('generates FEN from moves', (initialFen, moves, fen) => {
    expect(generateFenFromMoves(initialFen, moves)).toStrictEqual(fen)
  })

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
    [
      'info depth 30 seldepth 39 multipv 1 score cp -20 nodes 12336105 nps 4630670 hashfull 981 tbhits 0 time 2664 pv b8c6 f1b5 g8f6',
      {
        depth: '30',
        seldepth: '39',
        multipv: '1',
        score: {
          type: 'cp',
          value: '-20',
        },
        nodes: '12336105',
        nps: '4630670',
        hashfull: '981',
        tbhits: '0',
        time: '2664',
        pv: ['b8c6', 'f1b5', 'g8f6'],
      },
    ],

    
  ])('parses uci strings', (uci, expected) => {
    expect(parseUciString(uci)).toStrictEqual(expected)
  })
}
