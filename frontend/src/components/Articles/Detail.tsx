'use client'

import React, { useEffect } from 'react';
import { useArticle } from "./articles.queries";


interface DetailProps {
  slug: string;
}


const Detail: React.FC<DetailProps> = ({ slug }) => {
  const { data, isLoading, isError, error } = useArticle(slug);

  useEffect(() => {
    if (!isLoading &&!isError && data) {
      // hljs.highlightAll();
    }
  }, [isLoading, isError, data]);

  if (isLoading) {
    return <div>Loading...</div>; 
  }

  if (isError ||!data) {
    return <div>Error loading article: {error?.message}</div>; 
  }
  
  return (
    <div className="min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:pl-8 lg:pr-0 xl:px-16">
      <header className="mb-9 space-y-1">
        <h1 className="font-display text-3xl tracking-tight text-slate-900 dark:text-white">{data.title}</h1>
      </header>
      <div className="prose prose-slate max-w-none dark:prose-invert dark:text-slate-400 prose-headings:scroll-mt-28 prose-headings:font-display prose-headings:font-normal lg:prose-headings:scroll-mt-[8.5rem] prose-lead:text-slate-500 dark:prose-lead:text-slate-400 prose-a:font-semibold dark:prose-a:text-sky-400 prose-a:no-underline prose-a:shadow-[inset_0_-2px_0_0_var(--tw-prose-background,#fff),inset_0_calc(-1*(var(--tw-prose-underline-size,4px)+2px))_0_0_var(--tw-prose-underline,theme(colors.sky.300))] hover:prose-a:[--tw-prose-underline-size:6px] dark:[--tw-prose-background:theme(colors.slate.900)] dark:prose-a:shadow-[inset_0_calc(-1*var(--tw-prose-underline-size,2px))_0_0_var(--tw-prose-underline,theme(colors.sky.800))] dark:hover:prose-a:[--tw-prose-underline-size:6px] prose-pre:rounded-xl prose-pre:bg-slate-900 prose-pre:shadow-lg dark:prose-pre:bg-slate-800/60 dark:prose-pre:shadow-none dark:prose-pre:ring-1 dark:prose-pre:ring-slate-300/10 dark:prose-hr:border-slate-800" dangerouslySetInnerHTML={{ __html: data.content }} />
    </div>
  );
};

export default Detail;
