'use client'

import React, { useEffect, useState } from 'react';
import { useArticle } from "./articles.queries";
import { IArticle } from "./articles.api";

import { DocsLayout } from '@/components/DocsLayout'
import { collectSections } from '@/lib/sections'
import { nodeServerAppPaths } from 'next/dist/build/webpack/plugins/pages-manifest-plugin';


interface DetailProps {
  slug: string;
}

function extractHeadings(content: string): any[] {
  const parser = new DOMParser();
  const doc = parser.parseFromString(content, "text/html");
  const body = doc.body;
  let headingsTree: any[] = [];
  for (let node of body.childNodes) {
    if (node.nodeType === Node.ELEMENT_NODE && ['H2', 'H3'].includes(node.nodeName.toUpperCase())) {
      const title = node.textContent || '';
      const level = parseInt(node.nodeName[1]);
      const currentHeading = {
        type: 'heading',
        attributes: { level, content: title },
      };
      headingsTree.push({...currentHeading});
    }
  }
  return headingsTree;
}

const Detail: React.FC<DetailProps> = ({ slug }) => {
  const {  data, isLoading, isError, error } = useArticle(slug);

  useEffect(() => {
    if (!isLoading &&!isError && data) {
     
    }
  }, [isLoading, isError, data]);

  if (isLoading) {
    return <div>Loading...</div>; 
  }

  if (isError ||!data) {
    return <div>Error loading article: {error?.message}</div>; 
  }
  const nodes = extractHeadings(data.content);

  return (
    <DocsLayout
    children={<div dangerouslySetInnerHTML={{ __html: data.content }} />}
    frontmatter={{ title: data.title }}
    nodes={nodes}
  />
  );
};

export default Detail;
