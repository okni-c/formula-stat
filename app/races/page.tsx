'use client'

import { AnimatePresence, motion } from 'framer-motion'
import RacePageHeader from '../components/RacePageHeader'
import ListBlock from '../components/ListBlock'
import RaceBlock from '../components/RaceBlock'

export default function RacePage() {
    return (
        <AnimatePresence>
            <motion.main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{
                    duration: 0.1,
                    delay: 0.1
                }}>
                    <RacePageHeader heading="Up Next:" subtitle="Miami Qualifying" />
                    <ListBlock title={'Past Events'}>
                        <RaceBlock location={'Chinese GP'} winner={'Lewis Hamilton'} date={'Jul 10th, 2023'} flagcode={'cn'} />
                        <RaceBlock location={'Chinese GP'} winner={'Lewis Hamilton'} date={'Jul 10th, 2023'} flagcode={'cn'} />
                        <RaceBlock location={'Chinese GP'} winner={'Lewis Hamilton'} date={'Jul 10th, 2023'} flagcode={'cn'} />
                        <RaceBlock location={'Chinese GP'} winner={'Lewis Hamilton'} date={'Jul 10th, 2023'} flagcode={'cn'} />
                        <RaceBlock location={'Miami'} winner={'Lewis Hamilton'} date={'Jan 10th, 2023'} flagcode={'usa'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                        <RaceBlock location={'Japan GP'} winner={'Lewis Hamilton'} date={'Aug 10th, 2023'} flagcode={'jp'} />
                    </ListBlock>
            </motion.main>
        </AnimatePresence>
    )
}
