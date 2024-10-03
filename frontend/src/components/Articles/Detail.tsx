'use client'

import React, { useEffect } from 'react';
import { useArticle } from "./articles.queries";
import { DocsLayout } from '@/components/DocsLayout'
import { notFound } from "next/navigation"
import 'highlight.js/styles/github-dark.css';
import hljs from 'highlight.js/lib/core';
import rust from "highlight.js/lib/languages/rust";
import javascript  from "highlight.js/lib/languages/javascript";
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('rust', rust);

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
    if (data) {
      document.querySelectorAll("div.ql-code-block").forEach((el: Element) => {
        hljs.highlightElement(el as HTMLElement);
      });
    }
    
  }, [data]);

  if (isLoading) {
    return <div>Loading...</div>; 
  }

  if (isError || !data) {
    notFound();
  }
  const nodes = extractHeadings(data.content);

  return (
    <DocsLayout
      frontmatter={{ title: data.title, section_name: data.section_name }}
      nodes={nodes}
    >
      <div dangerouslySetInnerHTML={{ __html: data.content }} className="dark:bg-transparent dark:text-white" />
    </DocsLayout>
  );
};

export default Detail;
