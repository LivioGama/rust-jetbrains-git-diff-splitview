'use client'
import {ModalProvider} from '@/components/general/useModals'
import {CacheProvider} from '@chakra-ui/next-js'
import {ChakraProvider} from '@chakra-ui/react'
import {QueryClient, QueryClientProvider} from '@tanstack/react-query'
import {SingleChildrenProps} from 'models/interfaces'
import theme from 'ui/themes'
import PlausibleProvider from 'next-plausible'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      retryDelay: attemptIndex => Math.min(1000 * 2 ** attemptIndex, 30000),
    },
  },
})

const Providers = ({children}: SingleChildrenProps) => (
  <CacheProvider>
    <PlausibleProvider domain='app.anbiti.me'>
      <ChakraProvider theme={theme}>
        <QueryClientProvider client={queryClient}>
          <ModalProvider>{children}</ModalProvider>
        </QueryClientProvider>
      </ChakraProvider>
    </PlausibleProvider>
  </CacheProvider>
)

export default Providers
