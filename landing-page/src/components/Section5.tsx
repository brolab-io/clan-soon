import Image from "next/image";

export default function Section5() {
  return (
    <>
      <section className='bg-[#121638]'>
        <div className='container mx-auto p-3 my-10 grid xl:grid-cols-2 items-center justify-items-center mt-20'>
          <div className='text-center xl:text-start'>
            <div>
              <h2 className='text-4xl font-bold'>Making early-investment</h2>
              <h2 className='text-4xl font-bold'>in Web3 easily</h2>
            </div>
            <a
              href={process.env.NEXT_PUBLIC_APP_URL}
              target='_blank'
            >
              <button className='border-2 bg-white text-black tracking-widest text-sm px-4 py-2 rounded-full hover:bg-white/20 transition-colors duration-300 mt-4'>
                Download app
              </button>
            </a>
          </div>
          <div className='flex justify-center'>
            <Image
              src={"/assets/iphone.png"}
              alt='Phone'
              width={383}
              height={665}
              className='object-cover mt-[250px] 2xl:mt-0'
              priority
            />
          </div>
        </div>
      </section>
    </>
  );
}
