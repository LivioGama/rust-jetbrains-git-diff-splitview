'use client'
import {ModalProvider} from '@/components/general/useModals'
import {CacheProvider} from '@chakra-ui/next-js'
import {ChakraProvider, extendTheme} from '@chakra-ui/react'
import {QueryClient, QueryClientProvider} from '@tanstack/react-query'
import {useEffect, useState} from 'react'
import {SingleChildrenProps} from 'models/interfaces'
import colors from 'ui/colors'
import CustomButton from 'ui/CustomButton'
import CustomCard from 'ui/CustomCard'
import CustomContainer from 'ui/CustomContainer'
import CustomHeading from 'ui/customHeading'
import CustomInput from 'ui/CustomInput'
import {CustomMenu} from 'ui/CustomMenu'
import CustomSlider from 'ui/CustomSlider'
import CustomTable from 'ui/CustomTable'
import CustomTabs from 'ui/CustomTabs'
import CustomText from 'ui/customText'
import globalStyles from 'ui/globalStyles'
import PlausibleProvider from 'next-plausible'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      retryDelay: attemptIndex => Math.min(1000 * 2 ** attemptIndex, 30000),
    },
  },
})

const createTheme = () => {
  const config = {
    initialColorMode: 'dark',
    useSystemColorMode: false,
  }

  return extendTheme({
    config,
    styles: globalStyles,
    colors,
    components: {
      Heading: CustomHeading,
      Text: CustomText,
      Button: CustomButton,
      Container: CustomContainer,
      Input: CustomInput,
      Menu: CustomMenu,
      Slider: CustomSlider,
      Tabs: CustomTabs,
      Table: CustomTable,
      Card: CustomCard,
    },
  })
}

const Providers = ({children}: SingleChildrenProps) => {
  const [theme, setTheme] = useState<any>(null)

  useEffect(() => {
    setTheme(createTheme())
  }, [])

  if (!theme) {
    return <div>Loading...</div>
  }

  return (
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
}

export default Providers
