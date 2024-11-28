import Footer from "@/components/Footer";
import NavBar from "@/components/NavBar";
import "./globals.css";

export const metadata = {
  title: "SolClan",
  description: "On-Chain governance for your Business",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-[#0D102C] text-white">
        <NavBar />
        <main>{children}</main>
        <Footer />
      </body>
    </html>
  );
}
