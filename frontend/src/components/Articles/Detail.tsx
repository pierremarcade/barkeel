'use client'

import React, { useEffect, useState } from 'react';
import { useArticle } from "./articles.queries";
import { DocsLayout } from '@/components/DocsLayout'

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
    frontmatter={{ title: data.title, name: data.name }}
    nodes={nodes}
  />
  );
};

export default Detail;
