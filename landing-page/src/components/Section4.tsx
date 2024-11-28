import Image from "next/image";

export default function Section4() {
  return (
    <section>
      <div className='container mx-auto p-3 my-10'>
        <h2 className='text-center text-4xl font-bold'>Experience the Best</h2>
        <h2 className='text-center text-4xl font-bold text-purple-500'>
          Investment DAO
        </h2>
        <div className='border-l border-dashed '>
          <div className='grid xl:grid-cols-2 justify-center mt-36 mx-4  text-center xl:text-start'>
            <div className='flex'>
              <div className='w-6 h-6 bg-[#B18CFF] rounded-full ml-[-28px] pr-[24px] mr-[24px] mt-2'></div>
              <div>
                <h2 className='text-4xl font-bold'>Create/Join a Clan</h2>
                <div className='flex mt-[10px] '>
                  The create/join clan function is a feature on Solclan that
                  allows users to create or join a community of like-minded
                  investors with a shared interest in investing in Web3 projects
                </div>
              </div>
            </div>
            <div className='flex justify-center'>
              <Image
                src={"/assets/ezgif1.png"}
                alt='Iphone1'
                width={299}
                height={648}
                className='object-cover mt-6'
                priority
              />
            </div>
          </div>
          <div className='grid xl:grid-cols-2 justify-center mt-36 mx-4  text-center xl:text-start'>
            <div className='flex'>
              <div className='w-6 h-6 bg-[#B18CFF] rounded-full ml-[-28px] pr-[24px] mr-[24px] mt-2'></div>
              <div>
                <h2 className='text-4xl font-bold'>
                  Make Proposal to Invest in Early-Startup
                </h2>
                <div className='flex mt-[10px] '>
                  To use this function, users can submit a proposal to invest in
                  a particular early-stage startup by providing details such as
                  the project&apos;s name, website, team members, and any other
                  relevant information. Once the proposal is submitted, other
                  users can view and discuss the opportunity, as well as ask
                  questions and offer feedback.
                </div>
              </div>
            </div>
            <div className='flex justify-center'>
              <Image
                src={"/assets/ezgif2.png"}
                alt='Iphone1'
                width={299}
                height={648}
                className='object-cover mt-6'
                priority
              />
            </div>
          </div>
          <div className='grid xl:grid-cols-2 justify-center mt-36 mx-4  text-center xl:text-start'>
            <div className='flex'>
              <div className='w-6 h-6 bg-[#B18CFF] rounded-full ml-[-28px] pr-[24px] mr-[24px] mt-2'></div>
              <div>
                <h2 className='text-4xl font-bold'>Auto vesting plan</h2>
                <div className='flex mt-[10px] '>
                  Auto-vesting function is a feature that allows investors to
                  automatically vest their tokens over a specific period of
                  time. This means that the tokens are gradually released to the
                  investor&apos;s wallet, rather than being fully released all
                  at once.
                </div>
              </div>
            </div>
            <div className='flex justify-center'>
              <Image
                src={"/assets/ezgif3.png"}
                alt='Iphone1'
                width={299}
                height={648}
                className='object-cover mt-6'
                priority
              />
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
