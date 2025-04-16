import { ThemeProvider } from "./theme-provider";
import { TooltipProvider } from "./ui/tooltip";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <ThemeProvider attribute="class" defaultTheme="dark">
        <TooltipProvider delayDuration={0}>
            {children}
        </TooltipProvider>
    </ThemeProvider>
  )
}