import { Engine, useEnginesStore } from '../stores/engines'
import { useSettingsStore } from '../stores/settings'
import { useUserStore } from '../stores/user'

type LichessEngine = {
  id: string
  name: string
  userId: string
  maxThreads: number
  maxHash: number
  defaultDepth: number
  variants: string[]
  providerData: string
  clientSecret: string
}

export async function saveEngine(engine: Engine): Promise<Engine> {
  const settings = useSettingsStore()
  const user = useUserStore()
  const engines = useEnginesStore()

  engine.providerSecret = settings.providerSecret

  let result = await fetch(`${settings.lichessHost}/api/external-engine`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${user.token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      name: engine.name,
      maxThreads: engine.maxThreads,
      maxHash: engine.maxHash,
      defaultDepth: engine.defaultDepth,
      variants: engine.variants,
      providerSecret: engine.providerSecret,
    }),
  }).then<LichessEngine>((response) => response.json())

  engine.id = result.id
  engines.addEngine(engine)

  return Promise.resolve(engine)
}
