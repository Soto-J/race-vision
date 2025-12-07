import { PageHeader } from "../components/page-header";

export default function TrafficIndicator() {
  return (
    <div>
      <PageHeader
        id="traffic-indicator"
        title="Traffic Indicator"
        description="Give a warning/notification in multiclass sessions. When there is a faster class driver within 3 seconds behind you, then this window becomes active."
        vars={[]}
      />
    </div>
  );
}
