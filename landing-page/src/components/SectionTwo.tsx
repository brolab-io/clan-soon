import Image from "next/image";

export default function SectionTwo() {
  return (
    <section className='text-white container mx-auto relative 2xl:mt-0 xl:mt-[650px] lg:mt-[850px]'>
      <Image
        src={"/assets/bg-section2.png"}
        alt='Section2'
        width={1684}
        height={730}
        className='object-cover w-full'
      />
      <div className='absolute top-0 left-0 right-0 bottom-0 flex flex-col justify-center space-y-10'>
        <h3 className='text-center font-semibold text-3xl'>
          Invest with a community, not just a platform
        </h3>
        <div className='grid 2xl:grid-cols-3 gap-10'>
          <div className='flex flex-col justify-center items-center text-center space-y-3'>
            <Image
              src={"/assets/investor.png"}
              alt='Investor'
              width={120}
              height={120}
              className='object-cover'
            />
            <h4 className='font-semibold text-xl'>Investor</h4>
            <p className='px-10'>
              Investing with a community help small investors pool their
              resources together to invest in larger, high-potential projects
              that may have been otherwise out of reach for individual
              investors.
            </p>
          </div>
          <div className='flex flex-col justify-center items-center text-center space-y-3'>
            <Image
              src={"/assets/vc.png"}
              alt='Venture Capital'
              width={120}
              height={120}
              className='object-cover'
            />
            <h4 className='font-semibold text-xl'>Venture Capital</h4>
            <p className='px-10'>
              By allowing other investors to participate in the private sale,
              the VC firm can increase the amount of funding available to the
              startup, while also reducing their risk exposure
            </p>
          </div>
          <div className='flex flex-col justify-center items-center text-center space-y-3'>
            <Image
              src={"/assets/project-founder.png"}
              alt='Project Founder'
              width={120}
              height={120}
              className='object-cover'
            />
            <h4 className='font-semibold text-xl'>Project founder</h4>
            <p className='px-10'>
              Access to funding, community building opportunities, exposure to
              the Web3 community, and access to expertise and guidance from
              experienced investors.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
