
import Detail from "@/components/Articles/Detail";
import type { Metadata, ResolvingMetadata } from 'next'
import { getArticle } from "@/components/Articles/articles.api";
import React from 'react';

export async function generateMetadata(): Promise<Metadata> {
    const data = await getArticle("why-barkeel");
   
    return {
      title: data.title,
      description: data.description,
    }
  }

export default function Page() {
    return <Detail slug="why-barkeel" />;
}