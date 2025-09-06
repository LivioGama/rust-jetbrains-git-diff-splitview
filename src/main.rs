import {getSystemInstructionsFor} from '@/lib/helpers/personality'
import {streamText} from 'ai'
import {openai} from '@ai-sdk/openai'

export const runtime = 'edge'

export const POST = async (req: Request) => {
  const {messages, selectedType} = await req.json()

  const systemMessage = {role: 'system', content: getSystemInstructionsFor(selectedType)}
  const conversationMessages = Array.isArray(messages) ? messages : [{role: 'user', content: messages.trim()}]

  const result = streamText({
    model: openai('gpt-4.1'),
    messages: [systemMessage, ...conversationMessages],
    temperature: 0.5,
  })

  return result.toDataStreamResponse({
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    },
  })
}
