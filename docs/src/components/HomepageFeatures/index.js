import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';
import Link from '@docusaurus/Link';

const FeatureList = [
  {
    title: '簡単にアバターに実装',
    Svg: require('@site/static/img/cube-duotone.svg').default,
    description: (
      <>
        実装方法にModular Avatarを使用しており、<br />
        PrefabをアバターにD&Dするだけで実装できます。
      </>
    ),
    to: "/docs/creators/about_prefabs_in_Resources"
  },
  {
    title: '秒から年まで幅広く利用可能',
    Svg: require('@site/static/img/app-window-duotone.svg').default,
    description: (
      <>
        秒や分の他、午前/午後や日付も利用できます。<br />
        Float型やInt型の両方で利用可能な単位もあります。
      </>
    ),
    to: "/docs/creators/parameters"
  },
  {
    title: 'サンプル腕時計付き',
    Svg: require('@site/static/img/clock-duotone.svg').default,
    description: (
      <>
        どのように動作するか試してみてください！<br />
        有料ですか？いいえ、無料です！
      </>
    ),
    to: "/docs/users/sample_asset"
  },
];

function Feature({Svg, title, description,to}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
      <div>
          <center>
            <Link
              className="ToCenter button button--secondary button--lg"
              to={to}>
              詳しく見る
            </Link>
          </center>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
