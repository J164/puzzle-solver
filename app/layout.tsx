import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Nav from "./components/nav";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Puzzle Solver",
  description: "Website for creating, solving, and generating images of puzzles",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="h-screen flex flex-col">
          <Nav />
          {children}
          <footer className="flex items-center justify-center text-black text-sm mx-0 bg-gray-400 p-4 w-dvw">
            <p className="text-l">Author: Jacob Edley |</p>
            <a href="https://github.com/J164/puzzle-solver">
              <img src="github-logo.svg" alt="GitHub" className="inline-block w-8 h-8 mx-2" />
            </a>
            <a href="https://www.linkedin.com/in/jedley/">
              <img src="linkedin-logo.png" alt="LinkedIn" className="inline-block w-10 h-8 mx-2" />
            </a>
          </footer>
        </div>
      </body>
    </html>
  );
}
