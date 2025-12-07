import { PageHeader } from "../components/page-header";

export default function Standings() {
  return (
    <div>
      <PageHeader
        id="standings"
        title="Standings"
        description="This overlay shows the other drivers position and information that is relevant during a race, qualy or practice session. The info is customizable in the content section."
        vars={[]}
      />
    </div>
  );
}
