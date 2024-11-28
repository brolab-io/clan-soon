import Image from "next/image";
export default function Section3() {
  return (
    <section>
      <div className='container mx-auto px-3 mt-[550px] 2xl:mt-0'>
        <div className='grid 2xl:grid-cols-2 w-full'>
          <div className='space-y-3'>
            <h3 className='text-purple-500'>Investment DAO</h3>
            <h2 className='text-4xl uppercase tracking-wide font-bold'>
              OUR USP
            </h2>
            <p className='font-sans justify-start'>
              Solclan&apos;s USPs include on-chain governance, anti-rug
              protection, automated financial execution, and a mobile app. These
              features provide transparency, safety, efficiency, and convenience
              for users, making Solclan a trustworthy and user-friendly platform
              for small investors, venture capitalists, and project founders in
              the Web3 space.
            </p>
            <button className='bg-white text-black font-bold text-sm px-4 py-2 rounded-full hover:bg-white/90 transition-colors duration-300'>
              Get start now
            </button>
          </div>
          <div className='grid grid-cols-2 gap-5 px-5 max-w-full justify-center items-center mt-10 2xl:mt-0'>
            <div className='flex justify-end'>
              <div className='bg-white/5 rounded-sm p-5 flex flex-col justify-center items-center w-52 h-52 -mt-10  space-y-3'>
                <Image
                  src={"/assets/bitcoin-encryption.png"}
                  alt='bitcoin-encryption.png'
                  width={76}
                  height={76}
                  className='object-cover'
                  priority
                />
                <h4 className='font-bold text-center'>
                  On-chain <br />
                  governance
                </h4>
              </div>
            </div>
            <div className='flex justify-start'>
              <div className='bg-white/5 rounded-sm p-5 flex flex-col justify-center items-center w-52 h-52 mt-10 space-y-3'>
                <Image
                  src={"/assets/cryptocurrencies-going-up.png"}
                  alt='cryptocurrencies-going-up.png'
                  width={76}
                  height={76}
                  className='object-cover'
                  priority
                />
                <h4 className='font-bold text-center'>
                  Anti-rug
                  <br /> protection
                </h4>
              </div>
            </div>
            <div className='flex justify-end'>
              <div className='bg-white/5 rounded-sm p-5 flex flex-col justify-center items-center w-52 h-52 -mt-10  space-y-3'>
                <Image
                  src={"/assets/withdraw-a.png"}
                  alt='withdraw-a.png'
                  width={76}
                  height={76}
                  className='object-cover'
                  priority
                />
                <h4 className='font-bold text-center'>
                  Non-
                  <br />
                  custodial
                  <br /> platform
                </h4>
              </div>
            </div>
            <div className='flex justify-start'>
              <div className='bg-white/5 rounded-sm p-5 flex flex-col justify-center items-center w-52 h-52  space-y-3'>
                <Image
                  src={"/assets/bitcoin-wallet-a.png"}
                  alt='bitcoin-wallet-a.png'
                  width={76}
                  height={76}
                  className='object-cover'
                  priority
                />
                <h4 className='font-bold text-center'>
                  Automated
                  <br /> financial
                  <br /> execution
                </h4>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
