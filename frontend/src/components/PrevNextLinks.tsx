'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'

import {useMenus} from "@/components/Menus/menus.queries";

function ArrowIcon(props: React.ComponentPropsWithoutRef<'svg'>) {
  return (
    <svg viewBox="0 0 16 16" aria-hidden="true" {...props}>
      <path d="m9.182 13.423-1.17-1.16 3.505-3.505H3V7.065h8.517l-3.506-3.5L9.181 2.4l5.512 5.511-5.511 5.512Z" />
    </svg>
  )
}

function PageLink({
  label,
  slug,
  homepage,
  dir = 'next',
  ...props
}: Omit<React.ComponentPropsWithoutRef<'div'>, 'dir' | 'label'> & {
  label: string
  slug: string,
  homepage: boolean
  dir?: 'previous' | 'next'
  className?: string
}) {
  return (
    <div {...props}>
      <dt className="font-display text-sm font-medium text-slate-900 dark:text-white">
        {dir === 'next' ? 'Next' : 'Previous'}
      </dt>
      <dd className="mt-1">
        <Link
          href={homepage ? '/' : slug}
          className={clsx(
            'flex items-center gap-x-1 text-base font-semibold text-slate-500 hover:text-slate-600 dark:text-slate-400 dark:hover:text-slate-300',
            dir === 'previous' && 'flex-row-reverse',
          )}
        >
          {label}
          <ArrowIcon
            className={clsx(
              'h-4 w-4 flex-none fill-current',
              dir === 'previous' && '-scale-x-100',
            )}
          />
        </Link>
      </dd>
    </div>
  )
}

export function PrevNextLinks() {
  const { menus } = useMenus()
  let pathname = usePathname()
  let allLinks = menus.map(section => 
    section.items?.map(item => ({
      slug: item.slug,
      label: item.label,
      homepage: item.homepage
    }))
  ).flat();
  let linkIndex = allLinks.findIndex((link) => `/${link?.homepage === true ? '' : link?.slug}` === pathname)
  let previousPage = linkIndex > -1 ? allLinks[linkIndex - 1] : null
  let nextPage = linkIndex > -1 ? allLinks[linkIndex + 1] : null
  
  if (!nextPage && !previousPage) {
    return null
  }

  return (
    <dl className="mt-12 flex border-t border-slate-200 pt-6 dark:border-slate-800">
      {previousPage && <PageLink dir="previous" {...previousPage} />}
      {nextPage && <PageLink className="ml-auto text-right" {...nextPage} />}
    </dl>
  )
}