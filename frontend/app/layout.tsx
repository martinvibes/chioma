'use client';

import type { Metadata, Viewport } from 'next';
import './globals.css';

import { QueryProvider } from '@/lib/query/provider';
import { StoreHydrator } from '@/store/StoreHydrator';

import ErrorMonitoringProvider from '@/components/error/ErrorMonitoringProvider';
import NetworkStatusBanner from '@/components/error/NetworkStatusBanner';
import { ErrorProvider } from '@/components/error/ErrorProvider';

import PwaController from '@/components/pwa/PwaController';

import { ModalProvider } from '@/contexts/ModalContext';
import { ModalManager } from '@/components/modals';

import { OfflineIndicator } from '@/components/offline';
import { ToastProvider } from '@/components/ui';

import { Inter } from 'next/font/google';
import { RouteAnnouncer } from '@/components/accessibility/RouteAnnouncer';

export const viewport: Viewport = {
  themeColor: '#1d4ed8',
  colorScheme: 'dark',
  width: 'device-width',
  initialScale: 1,
  maximumScale: 5,
  userScalable: true,
  viewportFit: 'cover',
};

const inter = Inter({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-sans',
});

export const metadata: Metadata = {
  metadataBase: new URL(
    process.env.NEXT_PUBLIC_APP_URL || 'https://chioma-kappa.vercel.app',
  ),
  title: {
    default: 'Chioma — Blockchain-Powered Rentals',
    template: '%s | Chioma',
  },
  description:
    'Automated commissions, zero disputes. Connect with landlords and tenants on the Stellar network.',
  manifest: '/manifest.webmanifest',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={inter.variable}>
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="anonymous"
        />
        <link rel="dns-prefetch" href="https://images.unsplash.com" />
      </head>

      <body className="antialiased font-sans bg-linear-to-br from-slate-900 via-blue-900 to-slate-900">
        {/* Accessibility: skip link */}
        <a href="#main-content" className="skip-link">
          Skip to main content
        </a>

        <QueryProvider>
          <ModalProvider>
            <ErrorProvider>
              <StoreHydrator />
              <ErrorMonitoringProvider />
              <PwaController />
              <NetworkStatusBanner />
              <RouteAnnouncer />

              {/* Main content (a11y target) */}
              <div id="main-content" tabIndex={-1}>
                {children}
              </div>

              <ModalManager />
              <OfflineIndicator />
              <ToastProvider />
            </ErrorProvider>
          </ModalProvider>
        </QueryProvider>
      </body>
    </html>
  );
}