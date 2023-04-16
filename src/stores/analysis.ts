import { Chess, Move, parseUci } from 'chessops'
import { makeFen, parseFen } from 'chessops/fen'
import { makeSanAndPlay } from 'chessops/san'
import { defineStore } from 'pinia'

export interface LichessWorkEvent {
  id: number
  event: string
  windowLabel: string
  payload: {
    event: 'Sleep' | 'Status' | 'Uci'
    message: string
    analysis_request: false
  }
}

type ChessColor = 'w' | 'b'
type UciScoreType = 'cp' | 'mate'

interface UciDetails {
  depth?: string
  seldepth?: string
  multipv?: string
  scoreType?: UciScoreType
  scoreValue?: string
  scoreBound?: string
  nodes?: string
  nps?: string
  hashfull?: string
  tbhits?: string
  time?: string
  pv?: string[]
  bestmove?: string
  ponder?: string
  currmove?: string
  currmovenumber?: string
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

export const useAnalysisStore = defineStore('analysis', {
  state: () => ({
    status: '',
    request: {} as LichessAnalysisRequest,
    uci: {} as UciDetails,
    sleepDuration: 0,
  }),
  getters: {
    evaluation(state): string {
      return state.uci.scoreType && state.uci.scoreValue
        ? convertScoreToEvaluation(
            this.whoseTurn,
            state.uci.scoreType,
            state.uci.scoreValue
          )
        : ''
    },
    nodes: (state) =>
      state.uci.nodes ? convertNumberToUnitString(state.uci.nodes) : '',
    nps: (state) =>
      state.uci.nps ? convertNumberToUnitString(state.uci.nps) : '',
    fen: (state) =>
      generateFenFromMoves(
        state.request.work?.initialFen,
        state.request.work?.moves
      ),
    whoseTurn(): ChessColor {
      return this.fen.split(' ')[1] as ChessColor
    },
    time: (state) =>
      state.uci.time ? convertTimeToUnitString(state.uci.time) : '',
    hashUsage: (state) =>
      state.uci.hashfull ? convertHashfullToPercentage(state.uci.hashfull) : '',
  },
  actions: {
    handle(event: LichessWorkEvent) {
      this.sleepDuration = 0

      if (event.payload.event === 'Status') {
        this.status = event.payload.message
      } else if (event.payload.event === 'Sleep') {
        this.status = 'Sleeping'
        this.sleepDuration = parseInt(event.payload.message)
      } else if (event.payload.event === 'Uci') {
        this.uci = { ...this.uci, ...parseUciString(event.payload.message) }
      }

      if (event.payload.analysis_request) {
        this.request = event.payload.analysis_request
      }
    },
  },
})

function convertNumberToUnitString(value: string): string {
  const number = parseInt(value)

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
  let details = [
    ['depth', uci.match(/depth (\d+)/)?.[1]],
    ['seldepth', uci.match(/seldepth (\d+)/)?.[1]],
    ['multipv', uci.match(/multipv (\d+)/)?.[1]],
    ['scoreType', uci.match(/score (\w+)/)?.[1] as UciScoreType],
    ['scoreValue', uci.match(/score \w+ (-?\d+)/)?.[1]],
    ['scoreBound', uci.match(/(upperbound|lowerbound)/)?.[1]],
    ['nodes', uci.match(/nodes (\d+)/)?.[1]],
    ['nps', uci.match(/nps (\d+)/)?.[1]],
    ['hashfull', uci.match(/hashfull (\d+)/)?.[1]],
    ['tbhits', uci.match(/tbhits (\d+)/)?.[1]],
    ['time', uci.match(/time (\d+)/)?.[1]],
    ['pv', uci.match(/ pv (.+)/)?.[1].split(' ')],
    ['currmove', uci.match(/currmove (\w+)/)?.[1]],
    ['currmovenumber', uci.match(/currmovenumber (\d+)/)?.[1]],
    ['bestmove', uci.match(/bestmove (\w+)/)?.[1]],
    ['ponder', uci.match(/ponder (\w+)/)?.[1]],
  ].filter(([, value]) => value !== undefined) as [string, string][]

  return Object.fromEntries(details) as UciDetails
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

function convertScoreToEvaluation(
  whoseTurn: ChessColor,
  scoreType: UciScoreType,
  scoreValue: string
): string {
  let value = parseInt(scoreValue || '0')

  if (whoseTurn === 'b') {
    value = -value
  }

  if (scoreType === 'mate') {
    return `#${value}`
  }

  return (value > 0 ? '+' : '') + (value / 100).toFixed(2)
}

function convertTimeToUnitString(time: string): string {
  return parseInt(time) / 1000 + 's'
}

function convertHashfullToPercentage(hashfull: string): string {
  return `${parseInt(hashfull) / 10}%`
}

if (import.meta.vitest) {
  const { expect, test } = import.meta.vitest

  test.each([
    ['0', '0%'],
    ['100', '10%'],
    ['1000', '100%'],
    ['456', '45.6%'],
  ])(`converts hashfull to percentage`, (hashfull, percent) => {
    expect(convertHashfullToPercentage(hashfull)).toBe(percent)
  })

  test.each([
    ['123', '0.123s'],
    ['1234', '1.234s'],
    ['12345', '12.345s'],
  ])(`converts time to unit string`, (time, unitString) => {
    expect(convertTimeToUnitString(time)).toBe(unitString)
  })

  test.each([
    ['w' as ChessColor, 'cp' as UciScoreType, '123', '+1.23'],
    ['w' as ChessColor, 'cp' as UciScoreType, '-123', '-1.23'],
    ['w' as ChessColor, 'mate' as UciScoreType, '1', '#1'],
    ['w' as ChessColor, 'mate' as UciScoreType, '-1', '#-1'],
    ['b' as ChessColor, 'cp' as UciScoreType, '123', '-1.23'],
    ['b' as ChessColor, 'cp' as UciScoreType, '-123', '+1.23'],
    ['b' as ChessColor, 'mate' as UciScoreType, '1', '#-1'],
    ['b' as ChessColor, 'mate' as UciScoreType, '-1', '#1'],
  ])(
    'converts score to evaluation',
    (whoseTurn, scoreType, scoreValue, evaluation) => {
      expect(
        convertScoreToEvaluation(whoseTurn, scoreType, scoreValue)
      ).toStrictEqual(evaluation)
    }
  )

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
    ['100', '100'],
    ['1090', '1.1k'],
    ['800403', '800.4k'],
    ['3123456', '3.1M'],
    ['1789123456', '1.8B'],
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
      'info depth 30 currmove b1d2 currmovenumber 4',
      {
        depth: '30',
        currmove: 'b1d2',
        currmovenumber: '4',
      },
    ],
    [
      'info depth 26 seldepth 36 multipv 1 score cp 209 nodes 21219470 nps 4641178 hashfull 1000 tbhits 0 time 4572 pv d1b3 d8e7',
      {
        depth: '26',
        seldepth: '36',
        multipv: '1',
        scoreType: 'cp',
        scoreValue: '209',
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
        scoreType: 'cp',
        scoreValue: '-20',
        nodes: '12336105',
        nps: '4630670',
        hashfull: '981',
        tbhits: '0',
        time: '2664',
        pv: ['b8c6', 'f1b5', 'g8f6'],
      },
    ],
    [
      'info depth 28 seldepth 36 multipv 1 score cp -119 lowerbound nodes 27756518 nps 4392549 hashfull 1000 tbhits 0 time 6319 pv b1c3',
      {
        depth: '28',
        hashfull: '1000',
        multipv: '1',
        nodes: '27756518',
        nps: '4392549',
        pv: ['b1c3'],
        scoreType: 'cp',
        scoreValue: '-119',
        scoreBound: 'lowerbound',
        seldepth: '36',
        tbhits: '0',
        time: '6319',
      },
    ],
  ])('parses uci strings', (uci, expected) => {
    expect(parseUciString(uci)).toStrictEqual(expected)
  })
}
